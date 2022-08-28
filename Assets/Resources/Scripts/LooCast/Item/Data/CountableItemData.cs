namespace LooCast.Item.Data
{
    using LooCast.Data;

    public abstract class CountableItemData : ItemData
    {
        public IntDataReference MaxCount;
        public IntDataReference DefaultCount;
        public FloatDataReference Density;
        public FloatDataReference MinObjectScale;
        public FloatDataReference MaxObjectScale;
    }
}