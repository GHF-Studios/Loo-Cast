using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Weapon.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "ChargedPlasmaLauncherWeaponData", menuName = "Data/Weapon/ChargedPlasmaLauncherWeaponData", order = 0)]
    public sealed class ChargedPlasmaLauncherWeaponData : WeaponData
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
    } 
}
