namespace LooCast.Item
{
    using Data;
    using LooCast.Resource;

    public class ResourceItem : AmountableItem
    {
        public Resource Resource { get; protected set; }

        public ResourceItem(ResourceItemData data) : base(data)
        {
            Resource = data.Resource;
        }
    }
}