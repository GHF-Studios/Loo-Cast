using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Linq;

namespace LooCast.Generator
{
    public class Generators : MonoBehaviour
    {
        public List<Generator> generators;

        private void Start()
        {
            foreach (Generator generator in generators)
            {
                generator.Initialize();
            }
        }
    } 
}
