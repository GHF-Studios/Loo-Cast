﻿using UnityEngine;

namespace LooCast.Health.Data.Runtime
{
    using LooCast.Variable;
    using Attribute.Stat;

    [CreateAssetMenu(fileName = "PlayerHealthRuntimeData", menuName = "Data/Health/Runtime/PlayerHealthRuntimeData", order = 0)]
    public class PlayerHealthRuntimeData : ScriptableObject
    {
        public Stats Stats;

        public void Initialize(PlayerHealthData data)
        {
            MaxHealth = new FloatComputedVariable(data.BaseMaxHealth.Value);
            MaxHealth.AddPermanentMultiplier(Stats.HealthMultiplier);

            Health = new FloatVariable(MaxHealth.Value);

            RegenerationAmount = new FloatComputedVariable(data.BaseRegenerationAmount.Value);
            RegenerationAmount.AddPermanentMultiplier(Stats.HealthRegenrationMultiplier);

            RegenerationTime = new FloatComputedVariable(data.BaseRegenerationTime.Value);
            RegenerationTimer = new FloatVariable(0.0f);

            Defense = new IntComputedVariable(data.BaseDefense.Value);
            Defense.AddPermanentIncrease(Stats.DefenseIncrease);

            KnockbackResistance = new FloatComputedVariable(data.BaseKnockbackResistance.Value);
            KnockbackResistance.AddPermanentMultiplier(Stats.KnockbackResistanceMultiplier);

            IsAlive = new BoolVariable(true);

            DamageIndicatorPrefab = data.DamageIndicatorPrefab;

            Team = data.Team;
        }

        public FloatVariable Health;
        public FloatComputedVariable MaxHealth;
        public FloatComputedVariable RegenerationAmount;
        public FloatComputedVariable RegenerationTime;
        public FloatVariable RegenerationTimer;
        public IntComputedVariable Defense;
        public FloatComputedVariable KnockbackResistance;
        public BoolVariable IsAlive;
        public GameObject DamageIndicatorPrefab;
        public IHealth.TeamType Team;
    }
}