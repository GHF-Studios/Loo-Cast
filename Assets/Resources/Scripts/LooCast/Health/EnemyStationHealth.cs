using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health
{
    using Random;
    using Orb;
    using Attribute.Stat;
    using Data;

    public sealed class EnemyStationHealth : StationHealth
    {
        public EnemyStationHealthData Data;

        private float experienceDropChance;
        private float experienceDropAmount;
        private GameObject experienceOrbPrefab;

        public Stats Stats;

        private void Start()
        {
            Initialize(Data);

            experienceDropChance = Data.BaseExperienceDropChance.Value;
            experienceDropAmount = Data.BaseExperienceDropAmount.Value;
            experienceOrbPrefab = Data.ExperienceOrbPrefab;
        }

        public override void Kill()
        {
            base.Kill();

            if (!isAlive && Random.Range(0.0f, 1.0f) < experienceDropChance)
            {
                GameObject xpOrbObject = Instantiate(experienceOrbPrefab, transform.position, Quaternion.identity);
                xpOrbObject.transform.localScale *= 2.5f;
                ExperienceOrb xpOrb = xpOrbObject.GetComponent<ExperienceOrb>();
                xpOrb.Initialize();
                xpOrb.SetExperience(experienceDropAmount);
            }
        }

        public override void Damage(DamageInfo damageInfo)
        {
            if (Random.Range(0.0f, 1.0f) < 0.1f * Stats.RandomChanceMultiplier)
            {
                damageInfo.damage *= 5.0f;
            }

            base.Damage(damageInfo);
        }
    } 
}
