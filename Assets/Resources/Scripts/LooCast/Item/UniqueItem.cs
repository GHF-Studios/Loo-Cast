namespace LooCast.Item
{
    using Data;

    public abstract class UniqueItem : Item
    {
        #region Data
        public UniqueItemData UniqueItemData { get; private set; }
        #endregion

        public UniqueItem(UniqueItemData data) : base(data)
        {
            UniqueItemData = data;
        }
    }
}