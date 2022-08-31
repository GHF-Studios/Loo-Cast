namespace LooCast.Mission
{
    using LooCast.Station;

    public class ReputationMissionReward : MissionReward
    {
        public MissionProvider MissionProvider { get; private set; }
        public int ReputationReward { get; private set; }

        public CreditsMissionReward(MissionProvider missionProvider, int reputationReward) : base()
        {
            MissionProvider = missionProvider;
            ReputationReward = reputationReward;
        }

        public override void Apply()
        {
            MissionProvider.Reputation += ReputationReward;
        }
    }
}