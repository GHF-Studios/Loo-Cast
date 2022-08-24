using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public class ResourceItemObject : AmountableItemObject
    {
        [SerializeField] protected ResourceItemData data;

        public ResourceItem ResourceItem { get; protected set; }

        private void Start()
        {
            ResourceItem = new ResourceItem(data);
            Initialize(ResourceItem);
        }
    }
}