using System;
using System.IO;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Mod
{
    using Core;

    public class ModManager
    {
        #region Static Properties
        public static ModManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ModManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ModManager instance;
        #endregion

        #region Properties
        public Dictionary<string, Mod> LoadedMods => loadedMods;
        #endregion

        #region Fields
        private Dictionary<string, Mod> loadedMods;
        #endregion

        #region Methods
        internal void Initialize()
        {
            loadedMods = new Dictionary<string, Mod>();
            Mod exampleMod = new Mod("LooCast", "Loo Cast Base Mod", "The Loo Cast Base Mod", "0.4.9", "0.4.9", "Leslie-John Richardson", new string[0], new string[0], new string[] { "Player" });
            SaveMod(exampleMod);
            LoadAllMods();
            ValidateAllMods();
        }

        internal void OnPreInitialize()
        {
            Debug.Log($"[ModManager] Starting Pre-Initialization.");

            foreach (Mod loadedMod in loadedMods.Values)
            {
                loadedMod.OnPreInitialize();
            }

            Debug.Log($"[ModManager] Finished Pre-Initialization.");
        }
        
        internal void OnInitialize()
        {
            Debug.Log($"[ModManager] Starting Initialization.");
            
            foreach (Mod loadedMod in loadedMods.Values)
            {
                loadedMod.OnInitialize();
            }

            Debug.Log($"[ModManager] Finished Initialization.");
        }

        internal void OnPostInitialize()
        {
            Debug.Log($"[ModManager] Starting Post-Initialization.");

            foreach (Mod loadedMod in loadedMods.Values)
            {
                loadedMod.OnPostInitialize();
            }

            Debug.Log($"[ModManager] Finished Post-Initialization.");
        }

        internal void SaveMod(Mod mod)
        {
            string path = Path.Combine(mod.FolderPath, $"{mod.Name}.LCMOD");
            string json = JsonUtility.ToJson(mod, true);
            File.WriteAllText(path, json);
        }

        private void LoadMod(FileInfo modInfoFile)
        {
            if (modInfoFile.Exists)
            {
                string json = File.ReadAllText(modInfoFile.FullName);
                Mod mod = JsonUtility.FromJson<Mod>(json);
                loadedMods.Add(mod.Name, mod);
            }
            else
            {
                throw new FileNotFoundException($"[ModManager] Mod Info '{modInfoFile.Name}' could not be found!");
            }
        }

        private void LoadAllMods()
        {
            DirectoryInfo modsFolder = new DirectoryInfo(MainManager.ModsFolderPath);
            DirectoryInfo[] modFolders = modsFolder.GetDirectories();
            FileInfo[] modInfoFiles = new FileInfo[modFolders.Length];
            for (int i = 0; i < modFolders.Length; i++)
            {
                modInfoFiles[i] = new FileInfo(Path.Combine(modFolders[i].FullName, $"{modFolders[i].Name}.LCMOD"));
            }
            foreach (FileInfo modInfoFile in modInfoFiles)
            {
                LoadMod(modInfoFile);
            }
        }

        private void ValidateAllMods()
        {
            foreach (Mod loadedMod in loadedMods.Values)
            {
                loadedMod.Validate();
            }
        }
        #endregion
    }
}
