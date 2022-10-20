using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Attribute.Stat;
    using Variable;

    public struct UpgradeSet
    {
        public UpgradeSet()
        {
            
        }

        public UpgradeSet(Stats stats)
        {
            MovementSpeedMultiplier = stats.MovementSpeedMultiplier;
            //UNDEFINED = stats.UNDEFINED;
            //UNDEFINED = stats.UNDEFINED;
            EnergyMultiplier = stats.EnergyMultiplier;
            ArmorPenetrationIncrease = stats.ArmorPenetrationIncrease;
            //UNDEFINED = stats.UNDEFINED;
            RandomChanceMultiplier = stats.RandomChanceMultiplier;
            //UNDEFINED = stats.UNDEFINED;
            DamageReflection = stats.DamageReflection;
            EnergyRegenerationMultiplier = stats.EnergyRegenerationMultiplier;
            NegativeEventChanceMultiplier = stats.NegativeEventChanceMultiplier;
            EnergyConsumptionMultiplier = stats.EnergyConsumptionMultiplier;
            PositiveEventChanceMultiplier = stats.PositiveEventChanceMultiplier;
            ExperienceMultiplier = stats.ExperienceMultiplier;
            LevelExperienceMaxMultiplier = stats.LevelExperienceMaxMultiplier;
            DamageMultiplier = stats.DamageMultiplier;
            RangeMultiplier = stats.RangeMultiplier;
            ProjectileSpeedMultiplier = stats.ProjectileSpeedMultiplier;
            KnockbackMultiplier = stats.KnockbackMultiplier;
            //UNDEFINED = stats.UNDEFINED;
            //UNDEFINED = stats.UNDEFINED;
            AttackDelayMultiplier = stats.AttackDelayMultiplier;
            HealthRegenrationMultiplier = stats.HealthRegenrationMultiplier;
            ConsecutiveProjectileDelayMultiplier = stats.ConsecutiveProjectileDelayMultiplier;
            ShieldStrengthIncrease = stats.ShieldStrengthIncrease;
            DefenseIncrease = stats.DefenseIncrease;
            PiercingIncrease = stats.PiercingIncrease;
            ProjectileSizeMultiplier = stats.ProjectileSizeMultiplier;
            //UNDEFINED = stats.UNDEFINED;
            //UNDEFINED = stats.UNDEFINED;
            //UNDEFINED = stats.UNDEFINED;
            DurationMultiplier = stats.DurationMultiplier;
            HealthMultiplier = stats.HealthMultiplier;
            //UNDEFINED = stats.UNDEFINED;
        }

        public float MovementSpeedMultiplier;
        //public float UNDEFINED;
        //public float UNDEFINED;
        public float EnergyMultiplier;
        public int ArmorPenetrationIncrease;
        //public float UNDEFINED;
        public float RandomChanceMultiplier;
        //public float UNDEFINED;
        public float DamageReflection;
        public float EnergyRegenerationMultiplier;
        public float NegativeEventChanceMultiplier;
        public float EnergyConsumptionMultiplier;
        public float PositiveEventChanceMultiplier;
        public float ExperienceMultiplier;
        public float LevelExperienceMaxMultiplier;
        public float DamageMultiplier;
        public float RangeMultiplier;
        public float ProjectileSpeedMultiplier;
        public float KnockbackMultiplier;
        //public float UNDEFINED;
        //public float UNDEFINED;
        public float AttackDelayMultiplier;
        public float HealthRegenrationMultiplier;
        public float ConsecutiveProjectileDelayMultiplier;
        public int ShieldStrengthIncrease;
        public int DefenseIncrease;
        public int PiercingIncrease;
        public float ProjectileSizeMultiplier;
        //public float UNDEFINED;
        //public float UNDEFINED;
        //public float UNDEFINED;
        public float DurationMultiplier;
        public float HealthMultiplier;
        //public float UNDEFINED;
    }
}