using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Manager
{
    using Data.Runtime;
    
    public class GameManager : Manager
    {
        [SerializeField] private RuntimeSets RuntimeSets;

        private void Awake()
        {
            RuntimeSets.Initialize();
        }

        private void OnApplicationQuit()
        {
            RuntimeSets.Initialize();
        }
    }
}
