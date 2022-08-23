namespace LooCast.Item
{
    using LooCast.Resource;

    public abstract class ResourceItem : AmountableItem
    {
        public abstract Resource Resource { get; protected set; }

        public ResourceItem(string name, float maxAmount, float amount, Resource resource) : base(name, maxAmount, amount)
        {
            Resource = resource;
        }
    }
}