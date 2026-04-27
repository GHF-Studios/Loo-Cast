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
                FreezeRayWeaponItem = (FreezeRayWeaponItem)value;
                if (FreezeRayWeaponItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
                Refresh();
            }
        }
        public FreezeRayWeaponItem FreezeRayWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            SpriteRenderer = GetComponent<SpriteRenderer>();

            Initialize((FreezeRayWeaponItem)data.CreateItem());
        }

        public void Refresh()
        {
            
        }
    }
}