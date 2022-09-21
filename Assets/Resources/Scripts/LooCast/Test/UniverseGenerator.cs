using System;
using UnityEngine;

namespace LooCast.Test
{
    using LooCast.Universe;
    
    public class UniverseGenerator : MonoBehaviour
    {
        [SerializeField] private Universe.GenerationSettings generationSettings;
        [SerializeField] private int voidAmount;
        [SerializeField] private int universeSize;
        [SerializeField] private int filamentSize;
        [SerializeField] private int sectorSize;
        [SerializeField] private int regionSize;
        [SerializeField] private GameObject filamentPrefab;
        [SerializeField] private GameObject sectorPrefab;
        [SerializeField] private GameObject regionPrefab;

        public void Generate()
        {
            Universe.Generate
            (
                generationSettings, 
                universeSize, 
                voidAmount, 
                filamentSize, 
                sectorSize, 
                regionSize, 
                filamentPrefab, 
                sectorPrefab,
                regionPrefab
            );
        }

        public void Save()
        {
            Universe.Save();
        }

        public void Load()
        {
            Universe.Load();
        }

        public void Unload()
        {
            Universe.Unload();
        }
    }
}