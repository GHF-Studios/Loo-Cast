namespace LooCast.Item.Data
{
    using LooCast.Data;

    public abstract class AmountableItemData : ItemData
    {
        public FloatDataReference MaxAmount;
        public FloatDataReference DefaultAmount;
        public FloatDataReference Density;
        public FloatDataReference MinObjectScale;
        public FloatDataReference MaxObjectScale;
    }
}