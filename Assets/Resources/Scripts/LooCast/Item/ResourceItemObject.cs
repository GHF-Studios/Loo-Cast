using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public class ResourceItemObject : AmountableItemObject
    {
        [SerializeField] protected ResourceItemData data;

        public ResourceItem ResourceItem { get; protected set; }
        public Rigidbody2D Rigidbody2D { get; protected set; }

        private void Awake()
        {
            ResourceItem = new ResourceItem(data);
            Initialize(ResourceItem);

            Rigidbody2D = GetComponent<Rigidbody2D>();
            ResourceItem.OnAmountChanged.AddListener(() => { Refresh(); });

            Refresh();
        }

        public void Refresh()
        {
            Rigidbody2D.mass = ResourceItem.Amount * ResourceItem.Density;
        }
    }
}