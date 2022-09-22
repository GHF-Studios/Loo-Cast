using System;
using UnityEngine;

namespace LooCast.Test
{
    using LooCast.Universe;
    
    public class UniverseGenerator : MonoBehaviour
    {
        [SerializeField] private Universe.GenerationSettings DEV_UNIVERSE_universeGenerationSettings;

        [SerializeField] private Vector2Int DEV_FILAMENT_filamentPosition;
        [SerializeField] private Vector2Int[] DEV_FILAMENT_filamentPositions;

        [SerializeField] private Vector2Int DEV_SECTOR_filamentPosition;
        [SerializeField] private Vector2Int DEV_SECTOR_sectorPosition;
        [SerializeField] private Vector2Int[] DEV_SECTOR_sectorPositions;

        [SerializeField] private Vector2Int DEV_REGION_sectorPosition;
        [SerializeField] private Vector2Int DEV_REGION_regionPosition;
        [SerializeField] private Vector2Int[] DEV_REGION_regionPositions;

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

        #region Generation
        public void GenerateFilament()
        {
            Universe.Instance.GenerateFilament(DEV_FILAMENT_filamentPosition);
        }

        public void GenerateFilaments()
        {
            Universe.Instance.GenerateFilaments(DEV_FILAMENT_filamentPositions);
        }
        #endregion

        #region Saving
        public void SaveFilament()
        {
            Universe.Instance.SaveFilament(DEV_FILAMENT_filamentPosition);
        }

        public void SaveFilaments()
        {
            Universe.Instance.SaveFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void SaveAllFilaments()
        {
            Universe.Instance.SaveFilaments();
        }
        #endregion

        #region Loading
        public void LoadFilament()
        {
            Universe.Instance.LoadFilament(DEV_FILAMENT_filamentPosition);
        }

        public void LoadFilaments()
        {
            Universe.Instance.LoadFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void UnloadFilament()
        {
            Universe.Instance.UnloadFilament(DEV_FILAMENT_filamentPosition);
        }

        public void UnloadFilaments()
        {
            Universe.Instance.UnloadFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void UnloadAllFilaments()
        {
            Universe.Instance.UnloadFilaments();
        }
        #endregion

        #region Deletion
        public void DeleteFilament()
        {
            Universe.Instance.DeleteFilament(DEV_FILAMENT_filamentPosition);
        }

        public void DeleteFilaments()
        {
            Universe.Instance.DeleteFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void DeleteAllFilaments()
        {
            Universe.Instance.DeleteFilaments();
        }
        #endregion

        #region Spawning
        public void SpawnFilament()
        {
            Universe.Instance.SpawnFilament(DEV_FILAMENT_filamentPosition);
        }

        public void SpawnFilaments()
        {
            Universe.Instance.SpawnFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void DespawnFilament()
        {
            Universe.Instance.DespawnFilament(DEV_FILAMENT_filamentPosition);
        }

        public void DespawnFilaments()
        {
            Universe.Instance.DespawnFilaments(DEV_FILAMENT_filamentPositions);
        }
        #endregion

        #endregion

        #region Sector

        #region Generation
        public void GenerateSector()
        {
            Universe.Instance.GenerateSector(DEV_SECTOR_filamentPosition, DEV_SECTOR_sectorPosition);
        }

        public void GenerateSectors()
        {
            Universe.Instance.GenerateSectors(DEV_SECTOR_filamentPosition, DEV_SECTOR_sectorPositions);
        }
        #endregion

        #region Saving
        public void SaveSector()
        {
            Universe.Instance.SaveSector(DEV_SECTOR_sectorPosition);
        }

        public void SaveSectors()
        {
            Universe.Instance.SaveSectors(DEV_SECTOR_sectorPositions);
        }

        public void SaveAllSectors()
        {
            Universe.Instance.SaveSectors();
        }
        #endregion

        #region Loading
        public void LoadSector()
        {
            Universe.Instance.LoadSector(DEV_SECTOR_sectorPosition);
        }

        public void LoadSectors()
        {
            Universe.Instance.LoadSectors(DEV_SECTOR_sectorPositions);
        }

        public void UnloadSector()
        {
            Universe.Instance.UnloadSector(DEV_SECTOR_sectorPosition);
        }

        public void UnloadSectors()
        {
            Universe.Instance.UnloadSectors(DEV_SECTOR_sectorPositions);
        }

        public void UnloadAllSectors()
        {
            Universe.Instance.UnloadSectors();
        }
        #endregion

        #region Deletion
        public void DeleteSector()
        {
            Universe.Instance.DeleteSector(DEV_SECTOR_sectorPosition);
        }

        public void DeleteSectors()
        {
            Universe.Instance.DeleteSectors(DEV_SECTOR_sectorPositions);
        }

        public void DeleteAllSectors()
        {
            Universe.Instance.DeleteSectors();
        }
        #endregion

        #region Spawning
        public void SpawnSector()
        {
            Universe.Instance.SpawnSector(DEV_SECTOR_sectorPosition);
        }

        public void SpawnSectors()
        {
            Universe.Instance.SpawnSectors(DEV_SECTOR_sectorPositions);
        }

        public void DespawnSector()
        {
            Universe.Instance.DespawnSector(DEV_SECTOR_sectorPosition);
        }

        public void DespawnSectors()
        {
            Universe.Instance.DespawnSectors(DEV_SECTOR_sectorPositions);
        }
        #endregion

        #endregion

        #region Region

        #region Generation
        public void GenerateRegion()
        {
            Universe.Instance.GenerateRegion(DEV_REGION_sectorPosition, DEV_REGION_regionPosition);
        }

        public void GenerateRegions()
        {
            Universe.Instance.GenerateRegions(DEV_REGION_sectorPosition, DEV_REGION_regionPositions);
        }
        #endregion

        #region Saving
        public void SaveRegion()
        {
            Universe.Instance.SaveRegion(DEV_REGION_regionPosition);
        }

        public void SaveRegions()
        {
            Universe.Instance.SaveRegions(DEV_REGION_regionPositions);
        }

        public void SaveAllRegions()
        {
            Universe.Instance.SaveRegions();
        }
        #endregion

        #region Loading
        public void LoadRegion()
        {
            Universe.Instance.LoadRegion(DEV_REGION_regionPosition);
        }

        public void LoadRegions()
        {
            Universe.Instance.LoadRegions(DEV_REGION_regionPositions);
        }

        public void UnloadRegion()
        {
            Universe.Instance.UnloadRegion(DEV_REGION_regionPosition);
        }

        public void UnloadRegions()
        {
            Universe.Instance.UnloadRegions(DEV_REGION_regionPositions);
        }

        public void UnloadAllRegions()
        {
            Universe.Instance.UnloadRegions();
        }
        #endregion

        #region Deletion
        public void DeleteRegion()
        {
            Universe.Instance.DeleteRegion(DEV_REGION_regionPosition);
        }

        public void DeleteRegions()
        {
            Universe.Instance.DeleteRegions(DEV_REGION_regionPositions);
        }

        public void DeleteAllRegions()
        {
            Universe.Instance.DeleteRegions();
        }
        #endregion

        #region Spawning
        public void SpawnRegion()
        {
            Universe.Instance.SpawnRegion(DEV_REGION_regionPosition);
        }

        public void SpawnRegions()
        {
            Universe.Instance.SpawnRegions(DEV_REGION_regionPositions);
        }

        public void DespawnRegion()
        {
            Universe.Instance.DespawnRegion(DEV_REGION_regionPosition);
        }

        public void DespawnRegions()
        {
            Universe.Instance.DespawnRegions(DEV_REGION_regionPositions);
        }
        #endregion

        #endregion
    }
}