using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Health
{
    using Data;
    using Data.Runtime;
    using Sound;
    using UI.Screen;
    using Manager;
    using Random;

    [DisallowMultipleComponent]
    public class PlayerHealth : Health
    {
        public PlayerHealthData Data;
        public PlayerHealthRuntimeData RuntimeData;

        private GameSoundHandler soundHandler;
        private DeathScreen deathScreen;

        private void Start()
        {
            Initialize(Data);

            RuntimeData.Initialize(Data);

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

            RuntimeData.Health.Value -= damageInfo.damage;
            if (RuntimeData.Health.Value <= 0)
            {
                RuntimeData.Health.Value = 0;
                Kill();
            }
        }

        public override void Heal(float health)
        {
            RuntimeData.Health.Value += health;
            if (RuntimeData.Health.Value > RuntimeData.MaxHealth.Value)
            {
                RuntimeData.Health.Value = RuntimeData.MaxHealth.Value;
            }
        }

        public override void Kill()
        {
            if (RuntimeData.IsAlive.Value)
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
