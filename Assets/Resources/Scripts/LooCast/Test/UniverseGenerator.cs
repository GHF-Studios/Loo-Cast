using System;
using UnityEngine;

namespace LooCast.Test
{
    using LooCast.Universe;
    
    public class UniverseGenerator : MonoBehaviour
    {
        [SerializeField] private Universe.GenerationSettings DEV_UNIVERSE_universeGenerationSettings;

        [SerializeField] private Vector2Int DEV_FILAMENT_filamentPosition;

        [SerializeField] private Vector2Int DEV_SECTOR_filamentPosition;
        [SerializeField] private Vector2Int DEV_SECTOR_sectorPosition;

        [SerializeField] private Vector2Int DEV_REGION_sectorPosition;
        [SerializeField] private Vector2Int DEV_REGION_regionPosition;

        public bool DEV_UNIVERSE_showSection = false;
        public bool DEV_FILAMENT_showSection = false;
        public bool DEV_SECTOR_showSection = false;
        public bool DEV_REGION_showSection = false;

        #region Universe
        public void GenerateUniverse()
        {
            Universe.GenerateUniverse(DEV_UNIVERSE_universeGenerationSettings);
        }

        public void SaveUniverse()
        {
            Universe.SaveUniverse();
        }

        public void LoadUniverse()
        {
            Universe.LoadUniverse();
        }

        public void UnloadUniverse()
        {
            Universe.UnloadUniverse();
        }

        public void DeleteUniverse()
        {
            Universe.DeleteUniverse();
        }
        #endregion

        #region Filaments
        public void GenerateFilament()
        {
            Universe.Instance.GenerateFilament(DEV_FILAMENT_filamentPosition);
        }

        public void SaveFilament()
        {
            Universe.Instance.SaveFilament(DEV_FILAMENT_filamentPosition);
        }

        public void LoadFilament()
        {
            Universe.Instance.LoadFilament(DEV_FILAMENT_filamentPosition);
        }

        public void UnloadFilament()
        {
            Universe.Instance.LoadFilament(DEV_FILAMENT_filamentPosition);
        }

        public void DeleteFilament()
        {
            Universe.Instance.DeleteFilament(DEV_FILAMENT_filamentPosition);
        }

        public void SpawnFilament()
        {
            Universe.Instance.SpawnFilament(DEV_FILAMENT_filamentPosition);
        }

        public void DespawnFilament()
        {
            Universe.Instance.DespawnFilament(DEV_FILAMENT_filamentPosition);
        }
        #endregion

        #region Sector
        public void GenerateSector()
        {
            Universe.Instance.GenerateSector(DEV_SECTOR_filamentPosition, DEV_SECTOR_sectorPosition);
        }

        public void SaveSector()
        {
            Universe.Instance.SaveSector(DEV_SECTOR_sectorPosition);
        }

        public void LoadSector()
        {
            Universe.Instance.LoadSector(DEV_SECTOR_sectorPosition);
        }

        public void UnloadSector()
        {
            Universe.Instance.UnloadSector(DEV_SECTOR_sectorPosition);
        }

        public void DeleteSector()
        {
            Universe.Instance.DeleteSector(DEV_SECTOR_sectorPosition);
        }

        public void SpawnSector()
        {
            Universe.Instance.SpawnSector(DEV_SECTOR_sectorPosition);
        }

        public void DespawnSector()
        {
            Universe.Instance.DespawnSector(DEV_SECTOR_sectorPosition);
        }
        #endregion

        #region Region
        public void GenerateRegion()
        {
            Universe.Instance.GenerateRegion(DEV_REGION_sectorPosition, DEV_REGION_regionPosition);
        }

        public void SaveRegion()
        {
            Universe.Instance.SaveRegion(DEV_REGION_regionPosition);
        }

        public void LoadRegion()
        {
            Universe.Instance.LoadRegion(DEV_REGION_regionPosition);
        }

        public void UnloadRegion()
        {
            Universe.Instance.UnloadRegion(DEV_REGION_regionPosition);
        }

        public void DeleteRegion()
        {
            Universe.Instance.DeleteRegion(DEV_REGION_regionPosition);
        }

        public void SpawnRegion()
        {
            Universe.Instance.SpawnRegion(DEV_REGION_regionPosition);
        }

        public void DespawnRegion()
        {
            Universe.Instance.DespawnRegion(DEV_REGION_regionPosition);
        }
        #endregion
    }
}