using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;
    using LooCast.Attribute.Stat;

    public class FreezeRayWeaponItemObject : UniqueItemObject
    {
        [SerializeField] protected FreezeRayWeaponItemData data;

        public override Item Item
        {
            set
            {
                base.Item = value;
                FreezeRayWeaponItem = (FreezeRayWeaponItem)value;
                Refresh();
            }
        }
        public FreezeRayWeaponItem FreezeRayWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            //First we initialize independent members
            FreezeRayWeaponItem = new FreezeRayWeaponItem(data, this, stats);
            FreezeRayWeaponItem.OnSpawn.Invoke();
            SpriteRenderer = GetComponent<SpriteRenderer>();

            //Then we Initialize the ItemObject, as this sets the item and thus triggers Refresh, which needs to happen after Independent members have been initialized
            Initialize(FreezeRayWeaponItem);
        }

        public void Refresh()
        {
            
        }
    }
}