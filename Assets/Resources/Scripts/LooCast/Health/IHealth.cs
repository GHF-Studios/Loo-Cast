using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Health
{
    using LooCast.Variable;

    public interface IHealth
    {
        #region Enums
        public enum TeamType
        {
            Ally,
            Enemy,
            Neutral
        }
        #endregion

        #region Properties
        public FloatVariable Health { get; }
        public FloatComputedVariable MaxHealth { get; }
        public FloatComputedVariable RegenerationAmount { get; }
        public FloatComputedVariable RegenerationTime { get; }
        public FloatVariable RegenerationTimer { get; }
        public IntComputedVariable Defense { get; }
        public BoolVariable IsAlive { get; }
        public GameObject DamageIndicatorPrefab { get; }
        public TeamType Team { get; }
        #endregion

        #region Events
        UnityEvent OnKilled { get; }
        #endregion

        #region Methods
        void Damage(DamageInfo damageInfo);
        void IndicateDamage(DamageInfo damageInfo);
        void Heal(float health);
        void Kill();
        void Knockback(DamageInfo damageInfo);
        #endregion
    }
}
