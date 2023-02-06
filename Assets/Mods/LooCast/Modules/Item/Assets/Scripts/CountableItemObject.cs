using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class CountableItemObject : ItemObject
    {
        public CountableItem CountableItem { get; protected set; }
        public override Item Item
        {
            set
            {
                CountableItem = (CountableItem)value;
                if (CountableItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
            }
        }

        protected void Initialize(CountableItem countableItem)
        {
            base.Initialize(countableItem);
            CountableItem = countableItem;
        }
    }
}