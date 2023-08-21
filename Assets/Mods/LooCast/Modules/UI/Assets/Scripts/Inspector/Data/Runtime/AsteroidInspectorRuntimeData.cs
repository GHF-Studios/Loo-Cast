using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.UI.Inspector.Data.Runtime
{
    using LooCast.Asteroid;

    [CreateAssetMenu(fileName = "AsteroidInspectorRuntimeData", menuName = "Data/UI/Inspector/AsteroidInspectorRuntimeData", order = 0)]
    public class AsteroidInspectorRuntimeData : ScriptableObject
    {
        public Asteroid CurrentAsteroid
        {
            get
            {
                return currentAsteroid;
            }
            set
            {
                currentAsteroid = value;
                onCurrentAsteroidChanged.Invoke();
            }
        }
        private Asteroid currentAsteroid;
        public UnityEvent OnCurrentAsteroidChanged
        {
            get
            {
                return onCurrentAsteroidChanged;
            }
        }
        private UnityEvent onCurrentAsteroidChanged;

        private void OnEnable()
        {
            onCurrentAsteroidChanged = new UnityEvent();
        }
    } 
}
