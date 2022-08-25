using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;

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

            SpriteRenderer = GetComponent<SpriteRenderer>();

            Refresh();
        }

        public void Refresh()
        {
            Rigidbody2D.mass = ResourceItem.Amount * ResourceItem.Density;
            transform.localScale = Vector3.one * (ResourceItem.Amount / ResourceItem.MaxAmount).Map(0.0f, 1.0f, ResourceItem.MinObjectScale, ResourceItem.MaxObjectScale);
        }
    }
}