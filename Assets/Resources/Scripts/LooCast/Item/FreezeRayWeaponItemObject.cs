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
            SpriteRenderer = GetComponent<SpriteRenderer>();

            Initialize(FreezeRayWeaponItem);
        }

        public void Refresh()
        {
            
        }
    }
}