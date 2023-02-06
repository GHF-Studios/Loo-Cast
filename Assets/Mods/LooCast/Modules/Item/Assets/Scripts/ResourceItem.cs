using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Resource;

    public class ResourceItem : AmountableItem
    {
        #region Data
        public ResourceItemData ResourceItemData { get; private set; }
        #endregion

        public Resource Resource { get; protected set; }

        public ResourceItem(ResourceItemData data) : base(data)
        {
            ResourceItemData = data;

            Resource = data.Resource;
        }
    }
}