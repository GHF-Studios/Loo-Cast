using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Health
{
    using Data;
    using Data.Runtime;
    using Variable;
    using Sound;
    using UI.Screen;
    using UI.Canvas;
    using Attribute.Stat;
    using Manager;
    using Player;
    using Random;

    [RequireComponent(typeof(Player))]
    public class PlayerHealth : Health
    {
        public PlayerHealthData Data;
        public PlayerHealthRuntimeData RuntimeData;

        public Stats Stats;

        private GameSoundHandler soundHandler;
        private DeathScreen deathScreen;

        private void Start()
        {
            Initialize(Data);

            RuntimeData.MaxHealth = new FloatComputedVariable(Data.BaseMaxHealth.Value);
            RuntimeData.MaxHealth.AddPermanentMultiplier(Stats.HealthMultiplier);
            RuntimeData.Health = new FloatVariable(RuntimeData.MaxHealth.Value);
            RuntimeData.RegenerationAmount = new FloatComputedVariable(Data.BaseRegenerationAmount.Value);
            RuntimeData.RegenerationAmount.AddPermanentMultiplier(Stats.HealthRegenrationMultiplier);
            RuntimeData.RegenerationTime = new FloatComputedVariable(Data.BaseRegenerationTime.Value);
            RuntimeData.RegenerationTimer = new FloatVariable(0.0f);
            RuntimeData.Defense = new IntComputedVariable(Data.BaseDefense.Value);
            RuntimeData.Defense.AddPermanentIncrease(Stats.DefenseIncrease);
            RuntimeData.IsAlive = new BoolVariable(true);

            soundHandler = FindObjectOfType<GameSoundHandler>();
            deathScreen = FindObjectOfType<DeathScreen>();
        }

        protected override void OnPauseableUpdate()
        {
            Heal(RuntimeData.RegenerationAmount.Value * Time.deltaTime);
        }

        public override void Damage(DamageInfo damageInfo)
        {
            bool TryCriticalStrike(ref DamageInfo refDamageInfo)
            {
                if (Random.Range(0.0f, 1.0f) < refDamageInfo.critChance)
                {
                    refDamageInfo.damage = refDamageInfo.critDamage;
                    return true;
                }
                return false;
            }

            TryCriticalStrike(ref damageInfo);

            float defense = RuntimeData.Defense.Value;
            if (damageInfo.armorPenetration >= defense)
            {
                defense = 0;
            }
            else
            {
                defense -= damageInfo.armorPenetration;
            }

            damageInfo.damage -= defense;
            if (damageInfo.damage <= 0)
            {
                return;
            }

            RuntimeData.Health -= damageInfo.damage;
            if (RuntimeData.Health <= 0)
            {
                RuntimeData.Health = 0;
                Kill();
            }
        }

        public override void Heal(float health)
        {
            RuntimeData.Health += health;
            if (RuntimeData.Health > RuntimeData.MaxHealth.Value)
            {
                RuntimeData.Health = RuntimeData.MaxHealth.Value;
            }
        }

        public override void Kill()
        {
            if (RuntimeData.IsAlive)
            {
                base.Kill();
                GameSceneManager.Pause();
                soundHandler.SoundDeath();
                deathScreen.SetVisibility(true);
            }
        }

        public override void Knockback(DamageInfo damageInfo)
        {
            if (damageInfo.knockback != 0.0f)
            {
                Vector3 knockbackDirection = damageInfo.origin.transform.position - transform.position;
                Rigidbody2D rigidbody = GetComponent<Rigidbody2D>();
                if (rigidbody != null)
                {
                    rigidbody.AddForce(knockbackDirection.normalized * -250f * damageInfo.knockback, ForceMode2D.Impulse);
                }
            }
        }
    } 
}
