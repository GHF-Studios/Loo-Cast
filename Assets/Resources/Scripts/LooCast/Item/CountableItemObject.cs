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
                base.Item = value;
                CountableItem = (CountableItem)value;
            }
        }

        protected void Initialize(CountableItem countableItem)
        {
            base.Initialize(countableItem);
            CountableItem = countableItem;
        }
    }
}