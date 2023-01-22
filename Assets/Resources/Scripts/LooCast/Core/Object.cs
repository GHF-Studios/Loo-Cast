using System;
using UnityEngine;

namespace LooCast.Core
{
    public abstract class Object
    {
        public GameObject Instance;

        public Object()
        {
            Instance = new GameObject();
        }
    }
}
