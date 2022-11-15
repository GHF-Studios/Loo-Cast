using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Health
{
    using Data;
    using Data.Runtime;
    using LooCast.Core;
    using LooCast.Sound;
    using LooCast.UI.Canvas;
    using LooCast.UI.Screen;
    using LooCast.Game;
    using LooCast.Random;
    using LooCast.Indicator;
    using LooCast.Variable;
    using LooCast.Attribute.Stat;
    using static LooCast.Health.IHealth;

    [DisallowMultipleComponent]
    public class PlayerHealth : ExtendedMonoBehaviour, IHealth
    {
        #region Data
        public PlayerHealthData Data;
        public PlayerHealthRuntimeData RuntimeData;
        #endregion

        #region Properties
        public FloatVariable Health
        {
            get
            {
                return RuntimeData.Health;
            }
        }
        public FloatComputedVariable MaxHealth
        {
            get
            {
                return RuntimeData.MaxHealth;
            }
        }
        public FloatComputedVariable RegenerationAmount
        {
            get
            {
                return RuntimeData.RegenerationAmount;
            }
        }
        public FloatComputedVariable RegenerationTime
        {
            get
            {
                return RuntimeData.RegenerationTime;
            }
        }
        public FloatVariable RegenerationTimer
        {
            get
            {
                return RuntimeData.RegenerationTimer;
            }
        }
        public IntComputedVariable Defense
        {
            get
            {
                return RuntimeData.Defense;
            }
        }
        public FloatComputedVariable KnockbackResistance
        {
            get
            {
                return RuntimeData.KnockbackResistance;
            }
        }
        public BoolVariable IsAlive
        {
            get
            {
                return RuntimeData.IsAlive;
            }
        }
        public GameObject DamageIndicatorPrefab
        {
            get
            {
                return RuntimeData.DamageIndicatorPrefab;
            }
        }
        public TeamType Team
        {
            get
            {
                return RuntimeData.Team;
            }
        }
        #endregion

        #region Events
        public UnityEvent OnKilled
        {
            get
            {
                return onKilled;
            }

            set
            {
                onKilled = value;
            }
        }
        [SerializeField] private UnityEvent onKilled;
        #endregion

        #region Fields
        [SerializeField] private Stats stats;

        private GameSoundHandler soundHandler;
        private WorldSpaceCanvas canvas;
        private DeathScreen deathScreen;
        #endregion

        #region Methods
        private void Start()
        {
            RuntimeData.Initialize(Data);

            soundHandler = FindObjectOfType<GameSoundHandler>();
            canvas = FindObjectOfType<WorldSpaceCanvas>();
            deathScreen = FindObjectOfType<DeathScreen>();
        }

        protected override void PauseableUpdate()
        {
            Heal(RuntimeData.RegenerationAmount.Value * Time.deltaTime);
        }

        public void Damage(DamageInfo damageInfo)
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
            if (damageInfo.damage <= 0.0f)
            {
                return;
            }

            RuntimeData.Health.Value -= damageInfo.damage;
            if (RuntimeData.Health.Value <= 0.0f)
            {
                RuntimeData.Health.Value = 0.0f;
                Kill();
            }

            Knockback(damageInfo);
            IndicateDamage(damageInfo);
            soundHandler.SoundHit();
        }

        public void IndicateDamage(DamageInfo damageInfo)
        {
            Vector2 worldPos = new Vector2(transform.position.x + Random.Range(-0.5f, 0.5f), transform.position.y + Random.Range(-0.5f, 0.5f));
            GameObject damageIndicator = Instantiate(Data.DamageIndicatorPrefab, worldPos, Quaternion.identity, canvas.transform);
            damageIndicator.GetComponent<DamageIndicator>().Initialize(damageInfo.damage);
        }

        public void Heal(float health)
        {
            RuntimeData.Health.Value += health;
            if (RuntimeData.Health.Value > RuntimeData.MaxHealth.Value)
            {
                RuntimeData.Health.Value = RuntimeData.MaxHealth.Value;
            }
        }

        public void Kill()
        {
            if (RuntimeData.IsAlive.Value)
            {
                RuntimeData.IsAlive.Value = false;
                OnKilled.Invoke();
                GameManager.PauseGame();
                soundHandler.SoundDeath();
                deathScreen.SetVisibility(true);
            }
        }

        public void Knockback(DamageInfo damageInfo)
        {
            if (damageInfo.knockback != 0.0f)
            {
                if (KnockbackResistance.Value > 0.0f)
                {
                    damageInfo.knockback -= KnockbackResistance.Value;
                    if (damageInfo.knockback <= 0.0f)
                    {
                        damageInfo.knockback = 0.0f;
                    }
                }
                Vector3 knockbackDirection = damageInfo.origin.transform.position - transform.position;
                Rigidbody2D rigidbody = GetComponent<Rigidbody2D>();
                if (rigidbody != null)
                {
                    rigidbody.AddForce(knockbackDirection.normalized * -250f * damageInfo.knockback, ForceMode2D.Impulse);
                }
            }
        }
        #endregion
    } 
}
