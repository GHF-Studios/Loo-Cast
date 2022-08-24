namespace LooCast.Item
{
    using Data;

    public abstract class CountableItem : Item
    {
        public int MaxCount { get; protected set; }
        public int Count { get; protected set; }

        public CountableItem(CountableItemData data) : base(data)
        {
            MaxCount = data.MaxCount;
            Count = 0;
        }
    }
}