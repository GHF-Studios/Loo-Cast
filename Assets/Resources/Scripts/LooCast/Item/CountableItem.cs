namespace LooCast.Item
{
    using Data;

    public abstract class CountableItem : Item
    {
        public abstract int MaxCount { get; protected set; }
        public abstract int Count { get; protected set; }

        public CountableItem(string name, int maxCount, int count) : base(name)
        {
            MaxCount = maxCount;
            Count = count;
        }
    }
}