using UnityEngine;

namespace LooCast.Item.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "FreezeRayWeaponItemData", menuName = "Data/Item/Weapon/FreezeRayWeaponItemData", order = 0)]
    public class FreezeRayWeaponItemData : WeaponItemData
    {
        public override Item CreateItem()
        {
            return new FreezeRayWeaponItem(this, Stats, AutoFire.Value);
        }
    }
}