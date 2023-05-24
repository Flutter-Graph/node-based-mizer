import 'package:flutter/material.dart';
import 'package:mizer/available_nodes.dart';
import 'package:mizer/i18n.dart';
import 'package:mizer/protos/nodes.pb.dart';
import 'package:mizer/settings/hotkeys/hotkey_configuration.dart';
import 'package:mizer/state/nodes_bloc.dart';
import 'package:mizer/views/layout/layout_view.dart';
import 'package:mizer/views/nodes/widgets/node/base_node.dart';
import 'package:mizer/widgets/controls/button.dart';
import 'package:mizer/widgets/panel.dart';
import 'package:mizer/widgets/popup/popup_menu.dart';
import 'package:mizer/widgets/popup/popup_route.dart';
import 'package:provider/provider.dart';

import 'models/node_editor_model.dart';
import 'models/node_model.dart';
import 'widgets/editor_layers/canvas_drop_layer.dart';
import 'widgets/editor_layers/drag_selection_layer.dart';
import 'widgets/editor_layers/graph_paint_layer.dart';
import 'widgets/editor_layers/nodes_layer.dart';
import 'widgets/hidden_node_list.dart';
import 'widgets/properties/properties_pane.dart';

const double PathBreadcrumbHeight = 32;

class FetchNodesView extends StatelessWidget {
  const FetchNodesView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    context.read<NodesBloc>().add(FetchNodes());
    return SizedBox.expand(child: Consumer<NodeEditorModel>(builder: (context, model, _) {
      return NodesView(nodeEditorModel: model);
    }));
  }
}

class NodesView extends StatefulWidget {
  final NodeEditorModel nodeEditorModel;

  const NodesView({Key? key, required this.nodeEditorModel}) : super(key: key);

  @override
  _NodesViewState createState() => _NodesViewState();
}

class _NodesViewState extends State<NodesView> with WidgetsBindingObserver {
  Offset? addMenuPosition;
  bool showHiddenNodes = false;
  SelectionState? _selectionState;

  NodeEditorModel get model {
    return widget.nodeEditorModel;
  }

  @override
  Widget build(BuildContext context) {
    WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
      afterLayout(context);
    });

    return HotkeyConfiguration(
      hotkeySelector: (hotkeys) => hotkeys.nodes,
      hotkeyMap: {
        // TODO: determine position for new node
        "add_node": () => {},
        "duplicate_node": () => _duplicateNode(context),
        "delete_node": () => _deleteNode(context),
        "group_nodes": () => _groupNodes(context)
      },
      child: Row(
        children: [
          Expanded(
            child: Panel(
              label: "Nodes",
              child: Stack(
                children: [
                  GestureDetector(
                      onSecondaryTapUp: (event) {
                        Navigator.of(context).push(MizerPopupRoute(
                            position: event.globalPosition,
                            child: PopupMenu<Node_NodeType>(
                                categories: NODES.map((node) => node.category).toList(),
                                onSelect: (nodeType) => _addNode(model, nodeType))));
                        setState(() {
                          addMenuPosition = event.localPosition;
                        });
                      },
                      child: Overlay(initialEntries: [
                        OverlayEntry(
                          builder: (context) => Stack(children: [
                            SizedBox.expand(
                                child: InteractiveViewer(
                                    transformationController: model.transformationController,
                                    boundaryMargin: EdgeInsets.all(double.infinity),
                                    minScale: 0.1,
                                    maxScale: 10.0,
                                    child: SizedBox.expand())),
                            Transform(
                                transform: model.transformationController.value,
                                child: IgnorePointer(child: GraphPaintLayer(model: model))),
                            DragSelectionLayer(
                                nodes: model.nodes,
                                transformation: model.transformationController.value,
                                selectionState: _selectionState,
                                onUpdateSelection: (selection) =>
                                    setState(() => _selectionState = selection)),
                            CanvasDropLayer(),
                            NodesTarget(),
                            if (_selectionState != null) SelectionIndicator(_selectionState!),
                          ]),
                        )
                      ])),
                  Positioned(
                      bottom: 0,
                      right: 0,
                      left: 0,
                      height: PathBreadcrumbHeight,
                      child: Container(
                          color: Colors.grey.shade800,
                          child: Row(crossAxisAlignment: CrossAxisAlignment.end, children: [
                            MizerButton(
                                child: Padding(
                                  padding: const EdgeInsets.symmetric(horizontal: 8.0),
                                  child: Text("/"),
                                ),
                                onClick: () => model.closeContainer()),
                            ...model.path.map((p) => Center(child: Text(p)))
                          ])))
                ],
              ),
              actions: [
                PanelAction(
                    label: "Group Nodes".i18n,
                    onClick: () => _groupNodes(context),
                    hotkeyId: "group_nodes"),
                PanelAction(
                    label: "Delete Node".i18n,
                    disabled: model.selectedNode == null || !model.selectedNode!.node.canDelete,
                    onClick: () => _deleteNode(context),
                    hotkeyId: "delete_node"),
                PanelAction(
                    label: "Duplicate Node".i18n,
                    disabled: model.selectedNode == null || !model.selectedNode!.node.canDuplicate,
                    onClick: () => _duplicateNode(context),
                    hotkeyId: "duplicate_node"),
              ],
            ),
          ),
          Container(
            width: 256,
            child: Column(children: [
              Flexible(flex: 1, child: HiddenNodeList(nodes: model.hidden)),
              if (model.selectedNode != null)
                Flexible(
                    flex: 2,
                    child: Panel(
                        label: NODE_LABELS[model.selectedNode!.node.type] ?? "",
                        child: NodePropertiesPane(
                            node: model.selectedNode!.node, onUpdate: _refresh))),
            ]),
          )
        ],
      ),
    );
  }

  void _deleteNode(BuildContext context) {
    if (widget.nodeEditorModel.selectedNode != null &&
        widget.nodeEditorModel.selectedNode!.node.canDelete) {
      context.read<NodesBloc>().add(DeleteNode(widget.nodeEditorModel.selectedNode!.node.path));
    }
  }

  void _duplicateNode(BuildContext context) {
    if (widget.nodeEditorModel.selectedNode != null &&
        widget.nodeEditorModel.selectedNode!.node.canDuplicate) {
      context.read<NodesBloc>().add(DuplicateNode(widget.nodeEditorModel.selectedNode!.node.path,
          parent: widget.nodeEditorModel.parent?.node.path));
    }
  }

  void _groupNodes(BuildContext context) {
    List<NodeModel> nodes = [];
    if (widget.nodeEditorModel.selectedNode != null) {
      nodes.add(widget.nodeEditorModel.selectedNode!);
    }
    nodes.addAll(widget.nodeEditorModel.otherSelectedNodes);
    if (nodes.isEmpty) {
      return;
    }
    context.read<NodesBloc>().add(GroupNodes(nodes.map((n) => n.node.path).toList(),
        parent: widget.nodeEditorModel.parent?.node.path));
  }

  @override
  void initState() {
    WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
      WidgetsBinding.instance.addObserver(this);
      afterLayout(context);
    });
    super.initState();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  void afterLayout(BuildContext context) {
    model.updateNodes();
  }

  @override
  void didChangeMetrics() {
    super.didChangeMetrics();
    model.update();
  }

  void _addNode(NodeEditorModel model, Node_NodeType nodeType) {
    var transformedPosition = model.transformationController.toScene(addMenuPosition!);
    var position = transformedPosition / MULTIPLIER;
    context
        .read<NodesBloc>()
        .add(AddNode(nodeType: nodeType, position: position, parent: model.parent?.node.path));
    setState(() {
      addMenuPosition = null;
    });
  }

  void _refresh() {
    context.read<NodesBloc>().add(FetchNodes());
  }
}