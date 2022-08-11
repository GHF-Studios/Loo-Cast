using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Health
{
    using Data;
    using LooCast.Core;
    using LooCast.Variable;
    using LooCast.Sound;
    using LooCast.UI.Canvas;
    using LooCast.Indicator;
    using LooCast.Random;

    public class PlayerStationHealth : ExtendedMonoBehaviour, IHealth
    {
        #region Data
        public PlayerStationHealthData Data;
        #endregion

        #region Properties
        public FloatVariable Health { get; private set; }
        public FloatComputedVariable MaxHealth { get; private set; }
        public FloatComputedVariable RegenerationAmount { get; private set; }
        public FloatComputedVariable RegenerationTime { get; private set; }
        public FloatVariable RegenerationTimer { get; private set; }
        public IntComputedVariable Defense { get; private set; }
        public BoolVariable IsAlive { get; private set; }
        public GameObject DamageIndicatorPrefab { get; private set; }
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
        private GameSoundHandler soundHandler;
        private WorldSpaceCanvas canvas;
        #endregion

        #region Methods
        private void Start()
        {
            Health = new FloatVariable(Data.BaseMaxHealth.Value);
            MaxHealth = new FloatComputedVariable(Data.BaseMaxHealth.Value);
            RegenerationAmount = new FloatComputedVariable(Data.BaseRegenerationAmount.Value);
            RegenerationTime = new FloatComputedVariable(Data.BaseRegenerationTime.Value);
            RegenerationTimer = new FloatVariable(0.0f);
            Defense = new IntComputedVariable(Data.BaseDefense.Value);
            IsAlive = new BoolVariable(true);

            OnKilled = new UnityEvent();

            soundHandler = FindObjectOfType<GameSoundHandler>();
            canvas = FindObjectOfType<WorldSpaceCanvas>();
        }

        protected override void OnPauseableUpdate()
        {
            Heal(RegenerationAmount.Value * Time.deltaTime);
        }

        public void Damage(DamageInfo damageInfo)
        {
            float defense = Defense.Value;
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

            Health.Value -= damageInfo.damage;
            if (Health.Value <= 0.0f)
            {
                Health.Value = 0.0f;
                Kill();
            }

            Knockback(damageInfo);
            IndicateDamage(damageInfo);
            soundHandler.SoundHit();
        }

        public void IndicateDamage(DamageInfo damageInfo)
        {
            Vector2 worldPos = new Vector2(transform.position.x + Random.Range(-0.5f, 0.5f), transform.position.y + Random.Range(-0.5f, 0.5f));
            GameObject damageIndicator = Instantiate(Resources.Load<GameObject>("Prefabs/DamageIndicator"), worldPos, Quaternion.identity, canvas.transform);
            damageIndicator.GetComponent<DamageIndicator>().Initialize(damageInfo.damage);
        }

        public void Heal(float health)
        {
            Health.Value += health;
            if (Health.Value > MaxHealth.Value)
            {
                Health.Value = MaxHealth.Value;
            }
        }

        public void Kill()
        {
            if (IsAlive.Value)
            {
                IsAlive.Value = false;
                OnKilled.Invoke();
                soundHandler.SoundBigExplosion();
                Destroy(gameObject); 
            }
        }

        public void Knockback(DamageInfo damageInfo)
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
        #endregion
    }
}
