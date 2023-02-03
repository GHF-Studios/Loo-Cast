using UnityEngine;

namespace LooCast.Item.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "ChargedPlasmaLauncherWeaponItemData", menuName = "Data/Item/Weapon/ChargedPlasmaLauncherWeaponItemData", order = 0)]
    public class ChargedPlasmaLauncherWeaponItemData : WeaponItemData
    {
        public FloatDataReference ArcLifetime;
        public FloatDataReference ArcInitialWidth;
        public FloatDataReference ArcWidthMultiplier;
        public FloatDataReference ArcMinWidth;
        public IntDataReference ArcBranchAttempts;
        public FloatDataReference MinSpreadDistance;
        public FloatDataReference MinSpreadDistanceMultiplier;
        public FloatDataReference MaxSpreadDistance;
        public FloatDataReference MaxSpreadDistanceMultiplier;
        public FloatDataReference MinSpreadAngle;
        public FloatDataReference MinSpreadAngleMultiplier;
        public FloatDataReference MaxSpreadAngle;
        public FloatDataReference MaxSpreadAngleMultiplier;
        public FloatDataReference SpreadChance;
        public FloatDataReference SpreadChanceMultiplier;
        public FloatDataReference MinBranchDistance;
        public FloatDataReference MinBranchDistanceMultiplier;
        public FloatDataReference MaxBranchDistance;
        public FloatDataReference MaxBranchDistanceMultiplier;
        public FloatDataReference MinBranchAngle;
        public FloatDataReference MinBranchAngleMultiplier;
        public FloatDataReference MaxBranchAngle;
        public FloatDataReference MaxBranchAngleMultiplier;
        public FloatDataReference BranchChance;
        public FloatDataReference BranchChanceMultiplier;
        public IntDataReference MaxRecursionDepth;

        public override Item CreateItem()
        {
            return new ChargedPlasmaLauncherWeaponItem(this, Stats, AutoFire.Value);
        }
    }
}