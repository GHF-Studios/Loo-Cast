using UnityEngine;

namespace LooCast.Item.Data
{
    using LooCast.Resource;

    [CreateAssetMenu(fileName = "ResourceItemData", menuName = "Data/Item/Resource/ResourceItemData", order = 0)]
    public class ResourceItemData : AmountableItemData
    {
        public Resource Resource;
    }
}