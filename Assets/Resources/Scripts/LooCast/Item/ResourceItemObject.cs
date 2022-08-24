using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public class ResourceItemObject : AmountableItemObject
    {
        [SerializeField] protected ResourceItemData data;

        private void Start()
        {
            Item item = new ResourceItem(data);
            base.Initialize(item);
        }
    }
}