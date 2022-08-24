namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItem : Item
    {
        public float MaxAmount { get; protected set; }
        public float Amount { get; protected set; }

        public AmountableItem(AmountableItemData data) : base(data)
        {
            MaxAmount = data.MaxAmount;
            Amount = 0.0f;
        }
    }
}