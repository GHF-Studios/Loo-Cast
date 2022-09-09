namespace LooCast.Mission.Reward
{
    using LooCast.Station;

    public class ReputationMissionReward : MissionReward
    {
        public MissionProvider MissionProvider { get; private set; }
        public int ReputationReward { get; private set; }

        public ReputationMissionReward(MissionProvider missionProvider, int reputationReward) : base()
        {
            MissionProvider = missionProvider;
            ReputationReward = reputationReward;
        }

        public override void Reward()
        {
            MissionProvider.Reputation += ReputationReward;
        }
    }
}