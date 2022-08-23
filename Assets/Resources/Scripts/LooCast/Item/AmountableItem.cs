namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItem : Item
    {
        public abstract float MaxAmount { get; protected set; }
        public abstract float Amount { get; protected set; }

        public AmountableItem(string name, float maxAmount, float amount) : base(name)
        {
            MaxAmount = maxAmount;
            Amount = amount;
        }
    }
}