namespace LooCast.Item
{
    using Data;
    using System;
    using UnityEngine;

    public abstract class UniqueItem : Item
    {
        #region Data
        public UniqueItemData UniqueItemData { get; private set; }
        #endregion

        #region Properties
        public UniqueItemObject UniqueItemObject { get; private set; }
        #endregion

        public UniqueItem(UniqueItemData data) : base(data)
        {
            UniqueItemData = data;
        }

        public override void DropItem(Vector3 spawnPosition)
        {
            base.DropItem(spawnPosition);
            UniqueItemObject = (UniqueItemObject)ItemObject;
            if (UniqueItemObject == null)
            {
                throw new Exception("ItemObjectPrefab must contain a UniqueItemObject-component!");
            }
        }
    }
}