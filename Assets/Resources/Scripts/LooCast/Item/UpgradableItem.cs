using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Attribute.Stat;
    using LooCast.Item.Data;
    using Variable;

    public abstract class UpgradableItem : UniqueItem
    {
        #region Data
        public UpgradableItemData UpgradableItemData { get; private set; }
        #endregion

        #region Properties
        public UpgradableItemObject UpgradableItemObject { get; private set; }
        #endregion

        #region Fields
        protected Dictionary<int, Action> upgradeSetRemovementActions = new Dictionary<int, Action>();
        #endregion

        #region Constructors
        public UpgradableItem(UpgradableItemData data) : base(data)
        {
            UpgradableItemData = data;
        }
        #endregion

        #region Methods
        public override void DropItem(Vector3 spawnPosition)
        {
            base.DropItem(spawnPosition);
            UpgradableItemObject = (UpgradableItemObject)ItemObject;
            if (UpgradableItemObject == null)
            {
                throw new Exception("ItemObjectPrefab must contain a UpgradableItemObject-component!");
            }
        }

        public override void ContainItem(ItemContainer itemContainer)
        {
            base.ContainItem(itemContainer);
            IItemUpgrader itemUpgrader = itemContainer.OriginObject.GetComponentInChildren<IItemUpgrader>();
            if (itemUpgrader != null)
            {
                ApplyItemStatUpgradeSet(0, itemUpgrader.UpgradeSet);
            }
        }

        public abstract void ApplyItemStatUpgradeSet(int upgradeSetID, UpgradeSet upgradeSet);

        public void RemoveItemStatUpgradeSet(int upgradeSetID)
        {
            if (upgradeSetRemovementActions.ContainsKey(upgradeSetID))
            {
                upgradeSetRemovementActions.TryGetValue(upgradeSetID, out Action upgradeSetRemovementAction);
                upgradeSetRemovementAction.Invoke();
                upgradeSetRemovementActions.Remove(upgradeSetID);
            }
        }
        #endregion
    }
}