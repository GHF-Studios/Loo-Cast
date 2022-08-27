using UnityEngine;

namespace LooCast.Item.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "LaserEmitterWeaponItemData", menuName = "Data/Item/LaserEmitterWeaponItemData", order = 0)]
    public class LaserEmitterWeaponItemData : WeaponItemData
    {
        public FloatDataReference LaserLength;
    }
}