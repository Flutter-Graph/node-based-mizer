import 'package:mizer/protos/sequencer.pb.dart';

abstract class SequencerApi {
  Future<Sequences> getSequences();

  Future<Sequence> addSequence();

  Future<void> sequenceGo(int sequence);
}
