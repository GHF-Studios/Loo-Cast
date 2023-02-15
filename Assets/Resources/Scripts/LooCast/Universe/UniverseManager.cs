using UnityEngine;

namespace LooCast.Universe
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UniverseManager : ModuleManager
    {
        #region Static Properties
        public static UniverseManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UniverseManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<UniverseManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UniverseManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Universe", rootNamespace);
            looCastType = new Type(typeof(UniverseManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            #region Universe
            Type universeType = new Type(typeof(Universe), looCastNamespace);
            Type universeDensityMapGenerationUtilType = new Type(typeof(Universe.DensityMapGenerationUtil), looCastNamespace, universeType);
            Type universeMapElementLoadingUtilType = new Type(typeof(Universe.MapElementLoadingUtil), looCastNamespace, universeType);
            Type universeGenerationSettingsType = new Type(typeof(Universe.GenerationSettings), looCastNamespace, universeType);
            Type universeDensityMapType = new Type(typeof(Universe.DensityMap), looCastNamespace, universeType);

            typeManager.RegisterType(universeType);
            typeManager.RegisterType(universeDensityMapGenerationUtilType);
            typeManager.RegisterType(universeMapElementLoadingUtilType);
            typeManager.RegisterType(universeGenerationSettingsType);
            typeManager.RegisterType(universeDensityMapType);

            #region Universe.Object
            Type universeObjectType = new Type(typeof(Universe.Object), looCastNamespace, universeType);
            Type universeObjectTransformType = new Type(typeof(Universe.Object.Transform), looCastNamespace, universeObjectType);
            Type universeObjectTransformPositionType = new Type(typeof(Universe.Object.Transform.Position), looCastNamespace, universeObjectTransformType);

            typeManager.RegisterType(universeObjectType);
            typeManager.RegisterType(universeObjectTransformType);
            typeManager.RegisterType(universeObjectTransformPositionType);
            #endregion

            #region Universe.Filament
            Type universeFilamentType = new Type(typeof(Universe.Filament), looCastNamespace, universeType);
            Type universeFilamentGenerationSettingsType = new Type(typeof(Universe.Filament.GenerationSettings), looCastNamespace, universeFilamentType);
            Type universeFilamentPositionType = new Type(typeof(Universe.Filament.Position), looCastNamespace, universeFilamentType);

            typeManager.RegisterType(universeFilamentType);
            typeManager.RegisterType(universeFilamentGenerationSettingsType);
            typeManager.RegisterType(universeFilamentPositionType);

            #region Universe.Filament.Chunk
            Type universeFilamentChunkType = new Type(typeof(Universe.Filament.Chunk), looCastNamespace, universeFilamentType);
            Type universeFilamentChunkDensityMapTypeType = new Type(typeof(Universe.Filament.Chunk.DensityMapType), looCastNamespace, universeFilamentChunkType);
            Type universeFilamentChunkGenerationStateType = new Type(typeof(Universe.Filament.Chunk.GenerationState), looCastNamespace, universeFilamentChunkType);
            Type universeFilamentChunkPositionType = new Type(typeof(Universe.Filament.Chunk.Position), looCastNamespace, universeFilamentChunkType);
            Type universeFilamentChunkDensityMapType = new Type(typeof(Universe.Filament.Chunk.DensityMap), looCastNamespace, universeFilamentChunkType);
            Type universeFilamentChunkDensityMapCollectionType = new Type(typeof(Universe.Filament.Chunk.DensityMapCollection), looCastNamespace, universeFilamentChunkType);
            Type universeFilamentChunkDensityMapCoroutineInfoType = new Type(typeof(Universe.Filament.Chunk.DensityMapCoroutineInfo), looCastNamespace, universeFilamentChunkType);

            typeManager.RegisterType(universeFilamentChunkType);
            typeManager.RegisterType(universeFilamentChunkDensityMapTypeType);
            typeManager.RegisterType(universeFilamentChunkGenerationStateType);
            typeManager.RegisterType(universeFilamentChunkPositionType);
            typeManager.RegisterType(universeFilamentChunkDensityMapType);
            typeManager.RegisterType(universeFilamentChunkDensityMapCollectionType);
            typeManager.RegisterType(universeFilamentChunkDensityMapCoroutineInfoType);
            #endregion Universe.Filament.Chunk

            #endregion Universe.Filament

            #region Universe.Sector
            Type universeSectorType = new Type(typeof(Universe.Sector), looCastNamespace, universeType);
            Type universeSectorGenerationSettingsType = new Type(typeof(Universe.Sector.GenerationSettings), looCastNamespace, universeSectorType);
            Type universeSectorPositionType = new Type(typeof(Universe.Sector.Position), looCastNamespace, universeSectorType);

            typeManager.RegisterType(universeSectorType);
            typeManager.RegisterType(universeSectorGenerationSettingsType);
            typeManager.RegisterType(universeSectorPositionType);

            #region Universe.Sector.Chunk
            Type universeSectorChunkType = new Type(typeof(Universe.Sector.Chunk), looCastNamespace, universeSectorType);
            Type universeSectorChunkDensityMapTypeType = new Type(typeof(Universe.Sector.Chunk.DensityMapType), looCastNamespace, universeSectorChunkType);
            Type universeSectorChunkGenerationStateType = new Type(typeof(Universe.Sector.Chunk.GenerationState), looCastNamespace, universeSectorChunkType);
            Type universeSectorChunkPositionType = new Type(typeof(Universe.Sector.Chunk.Position), looCastNamespace, universeSectorChunkType);
            Type universeSectorChunkDensityMapType = new Type(typeof(Universe.Sector.Chunk.DensityMap), looCastNamespace, universeSectorChunkType);
            Type universeSectorChunkDensityMapCollectionType = new Type(typeof(Universe.Sector.Chunk.DensityMapCollection), looCastNamespace, universeSectorChunkType);
            Type universeSectorChunkDensityMapCoroutineInfoType = new Type(typeof(Universe.Sector.Chunk.DensityMapCoroutineInfo), looCastNamespace, universeSectorChunkType);

            typeManager.RegisterType(universeSectorChunkType);
            typeManager.RegisterType(universeSectorChunkDensityMapTypeType);
            typeManager.RegisterType(universeSectorChunkGenerationStateType);
            typeManager.RegisterType(universeSectorChunkPositionType);
            typeManager.RegisterType(universeSectorChunkDensityMapType);
            typeManager.RegisterType(universeSectorChunkDensityMapCollectionType);
            typeManager.RegisterType(universeSectorChunkDensityMapCoroutineInfoType);
            #endregion Universe.Sector.Chunk

            #endregion Universe.Sector

            #region Universe.Region
            Type universeRegionType = new Type(typeof(Universe.Region), looCastNamespace, universeType);
            Type universeRegionGenerationSettingsType = new Type(typeof(Universe.Region.GenerationSettings), looCastNamespace, universeRegionType);
            Type universeRegionPositionType = new Type(typeof(Universe.Region.Position), looCastNamespace, universeRegionType);

            typeManager.RegisterType(universeRegionType);
            typeManager.RegisterType(universeRegionGenerationSettingsType);
            typeManager.RegisterType(universeRegionPositionType);

            #region Universe.Region.Chunk
            Type universeRegionChunkType = new Type(typeof(Universe.Region.Chunk), looCastNamespace, universeRegionType);
            Type universeRegionChunkDensityMapTypeType = new Type(typeof(Universe.Region.Chunk.DensityMapType), looCastNamespace, universeRegionChunkType);
            Type universeRegionChunkGenerationStateType = new Type(typeof(Universe.Region.Chunk.GenerationState), looCastNamespace, universeRegionChunkType);
            Type universeRegionChunkPositionType = new Type(typeof(Universe.Region.Chunk.Position), looCastNamespace, universeRegionChunkType);
            Type universeRegionChunkDensityMapType = new Type(typeof(Universe.Region.Chunk.DensityMap), looCastNamespace, universeRegionChunkType);
            Type universeRegionChunkDensityMapCollectionType = new Type(typeof(Universe.Region.Chunk.DensityMapCollection), looCastNamespace, universeRegionChunkType);
            Type universeRegionChunkDensityMapCoroutineInfoType = new Type(typeof(Universe.Region.Chunk.DensityMapCoroutineInfo), looCastNamespace, universeRegionChunkType);

            typeManager.RegisterType(universeRegionChunkType);
            typeManager.RegisterType(universeRegionChunkDensityMapTypeType);
            typeManager.RegisterType(universeRegionChunkGenerationStateType);
            typeManager.RegisterType(universeRegionChunkPositionType);
            typeManager.RegisterType(universeRegionChunkDensityMapType);
            typeManager.RegisterType(universeRegionChunkDensityMapCollectionType);
            typeManager.RegisterType(universeRegionChunkDensityMapCoroutineInfoType);
            #endregion Universe.Region.Chunk

            #endregion Universe.Region

            #endregion Universe

            #endregion Namespace/Type/Instance Registration
        }
        #endregion
    }
}