using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Variable
{
    public abstract class ComputedVariable<T>
    {
        public readonly bool IsInitialized = false;
        public T Value
        {
            get
            {
                return ValueEvaluator.Invoke(PermanentMultipliers, PermanentIncreases, ActiveMultipliers, ActiveIncreases, BaseValue);
            }
        }
        public T BaseValue
        {
            get
            {
                return baseValue;
            }

            set
            {
                baseValue = value;
                if (IsInitialized)
                {
                    OnValueChanged.Invoke();
                }
            }
        }
        private T baseValue;
        public Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, T, T> ValueEvaluator { get; protected set; }
        public List<Multiplier> PermanentMultipliers { get; protected set; }
        public List<Increase> PermanentIncreases { get; protected set; }
        public List<TemporaryMultiplier> ActiveMultipliers { get; protected set; }
        public List<TemporaryIncrease> ActiveIncreases { get; protected set; }
        public UnityEvent OnValueChanged { get; protected set; }

        public ComputedVariable(T baseValue, Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, T, T> valueEvaluator)
        {
            IsInitialized = true;
            BaseValue = baseValue;
            ValueEvaluator = valueEvaluator;
            PermanentMultipliers = new List<Multiplier>();
            PermanentIncreases = new List<Increase>();
            ActiveMultipliers = new List<TemporaryMultiplier>();
            ActiveIncreases = new List<TemporaryIncrease>();
            OnValueChanged = new UnityEvent();
        }

        public static T[] Evaluate(ComputedVariable<T>[] computedValueVariables)
        {
            T[] evaluatedValues = new T[computedValueVariables.Length];
            for (int i = 0; i < computedValueVariables.Length; i++)
            {
                evaluatedValues[i] = computedValueVariables[i].Value;
            }
            return evaluatedValues;
        }

        public TemporaryMultiplier AddTimedMultiplier(float multiplier, float duration)
        {
            TemporaryMultiplier newMultiplier = new TemporaryMultiplier(multiplier, duration);
            ActiveMultipliers.Add(newMultiplier);
            OnValueChanged.Invoke();
            newMultiplier.OnTimerElapsed.AddListener(() => { ActiveMultipliers.Remove(newMultiplier); OnValueChanged.Invoke(); });
            return newMultiplier;
        }

        public TemporaryIncrease AddTimedIncrease(int increase, float duration)
        {
            TemporaryIncrease newIncrease = new TemporaryIncrease(increase, duration);
            ActiveIncreases.Add(newIncrease);
            OnValueChanged.Invoke();
            newIncrease.OnTimerElapsed.AddListener(() => { ActiveIncreases.Remove(newIncrease); OnValueChanged.Invoke(); });
            return newIncrease;
        }

        public Multiplier AddPermanentMultiplier(float multiplier)
        {
            Multiplier newMultiplier = new Multiplier(multiplier);
            PermanentMultipliers.Add(newMultiplier);
            return newMultiplier;
        }

        public Increase AddPermanentIncrease(int increase)
        {
            Increase newIncrease = new Increase(increase);
            PermanentIncreases.Add(newIncrease);
            return newIncrease;
        }
    }
}
