import 'dart:developer';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:mizer/protos/nodes.pb.dart';
import 'package:mizer/views/nodes/models/port_model.dart';

import 'node_model.dart';

class NodeEditorModel extends ChangeNotifier {
  final List<NodeModel> nodes;
  final List<NodeConnection> channels;
  final GlobalKey painterKey = GlobalKey();
  TransformationController transformationController;
  NodeModel selectedNode;

  NodeEditorModel(Nodes nodes)
      : this.nodes = nodes.nodes
            .map((node) => NodeModel(node: node, key: GlobalKey(debugLabel: "Node ${node.path}")))
            .toList(),
        this.channels = nodes.channels {
    transformationController = TransformationController()..addListener(update);
  }

  void updateNodes() {
    for (var node in nodes) {
      node.update(painterKey);
      node.updatePorts(painterKey);
    }
  }

  void update() {
    notifyListeners();
  }

  PortModel getPortModel(Node node, Port port, bool input) {
    var nodeModel = this.nodes.firstWhere((nodeModel) => nodeModel.node == node);

    return nodeModel.ports
        .firstWhere((portModel) => portModel.port == port && portModel.input == input);
  }

  Node getNode(String path) {
    return nodes.firstWhere((nodeModel) => nodeModel.node.path == path).node;
  }

  selectNode(NodeModel nodeModel) {
    this.selectedNode = nodeModel;
    this.update();
  }
}
