using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;

    public class ResourceItemObject : AmountableItemObject
    {
        [SerializeField] protected ResourceItemData data;

        public override Item Item
        {
            set
            {
                ResourceItem = (ResourceItem)value;
                if (ResourceItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
                Refresh();
            }
        }
        public ResourceItem ResourceItem { get; protected set; }
        public Rigidbody2D Rigidbody2D { get; protected set; }

        private void Awake()
        {
            //First we initialize independent members
            Rigidbody2D = GetComponent<Rigidbody2D>();
            SpriteRenderer = GetComponent<SpriteRenderer>();

            //Then we Initialize the ItemObject, as this sets the item and thus triggers Refresh, which needs to happen after Independent members have been initialized
            Initialize((ResourceItem)data.CreateItem());

            //Lastly we add Listeners for Refreshing when neccessary
            ResourceItem.OnAmountChanged.AddListener(() => { Refresh(); });
            OnItemChanged.AddListener(() => { Refresh(); });
        }

        public void Refresh()
        {
            Rigidbody2D.mass = ResourceItem.Amount * ResourceItem.Density;
            transform.localScale = Vector3.one * (ResourceItem.Amount / ResourceItem.MaxAmount).Map(0.0f, 1.0f, ResourceItem.MinObjectScale, ResourceItem.MaxObjectScale);
        }
    }
}