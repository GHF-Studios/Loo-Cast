namespace LooCast.Mission
{
    using LooCast.Currency;

    public class CreditsMissionReward : MissionReward
    {
        public Credits Credits { get; private set; }
        public int CreditsReward { get; private set; }

        public CreditsMissionReward(Credits credits, int creditsReward) : base()
        {
            Credits = credits;
            CreditsReward = creditsReward;
        }

        public override void Apply()
        {
            Credits.Balance += CreditsReward;
        }
    }
}