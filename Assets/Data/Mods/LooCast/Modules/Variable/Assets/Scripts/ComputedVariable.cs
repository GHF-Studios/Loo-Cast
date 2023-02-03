using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Variable
{
    public abstract class ComputedVariable<T>
    {
        public T Value
        {
            get
            {
                if (IsInitialized)
                {
                    return ValueEvaluator.Invoke(PermanentMultipliers, PermanentIncreases, TemporaryMultipliers, TemporaryIncreases, BaseValue);
                }
                return default(T);
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
        [HideInInspector, SerializeField] private T baseValue;
        public Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, T, T> ValueEvaluator { get; protected set; }
        public List<Multiplier> PermanentMultipliers { get; protected set; }
        public List<Increase> PermanentIncreases { get; protected set; }
        public List<TemporaryMultiplier> TemporaryMultipliers { get; protected set; }
        public List<TemporaryIncrease> TemporaryIncreases { get; protected set; }
        public UnityEvent OnValueChanged { get; protected set; }
        public readonly bool IsInitialized = false;

        public ComputedVariable(T baseValue, Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, T, T> valueEvaluator)
        {
            OnValueChanged = new UnityEvent();
            BaseValue = baseValue;
            ValueEvaluator = valueEvaluator;
            PermanentMultipliers = new List<Multiplier>();
            PermanentIncreases = new List<Increase>();
            TemporaryMultipliers = new List<TemporaryMultiplier>();
            TemporaryIncreases = new List<TemporaryIncrease>();
            IsInitialized = true;
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

        public TemporaryMultiplier AddTemporaryMultiplier(float multiplier, float duration)
        {
            TemporaryMultiplier newMultiplier = new TemporaryMultiplier(multiplier, duration);
            TemporaryMultipliers.Add(newMultiplier);
            OnValueChanged.Invoke();
            newMultiplier.OnTimerElapsed.AddListener(() => { TemporaryMultipliers.Remove(newMultiplier); OnValueChanged.Invoke(); });
            return newMultiplier;
        }

        public TemporaryIncrease AddTemporaryIncrease(int increase, float duration)
        {
            TemporaryIncrease newIncrease = new TemporaryIncrease(increase, duration);
            TemporaryIncreases.Add(newIncrease);
            OnValueChanged.Invoke();
            newIncrease.OnTimerElapsed.AddListener(() => { TemporaryIncreases.Remove(newIncrease); OnValueChanged.Invoke(); });
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

        public void RemoveTemporaryMultiplier(TemporaryMultiplier temporaryMultiplier)
        {
            TemporaryMultipliers.Remove(temporaryMultiplier); OnValueChanged.Invoke();
        }

        public void RemoveTemporaryIncrease(TemporaryIncrease temporaryIncrease)
        {
            TemporaryIncreases.Remove(temporaryIncrease); OnValueChanged.Invoke();
        }

        public void RemovePermanentMultiplier(Multiplier multiplier)
        {
            PermanentMultipliers.Remove(multiplier); OnValueChanged.Invoke();
        }

        public void RemovePermanentIncrease(Increase increase)
        {
            PermanentIncreases.Remove(increase); OnValueChanged.Invoke();
        }
    }
}
