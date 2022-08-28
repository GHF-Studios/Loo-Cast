using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Resource;

    public class ResourceItem : AmountableItem
    {
        public Resource Resource { get; protected set; }

        public ResourceItem(ResourceItemData data, ItemObject itemObject = null) : base(data, itemObject)
        {
            Resource = data.Resource;
        }
    }
}