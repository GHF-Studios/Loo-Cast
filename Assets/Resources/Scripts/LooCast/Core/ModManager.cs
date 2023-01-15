using System;
using System.IO;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Core
{
    public class ModManager
    {
        #region Static properties
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
        public static string ModsFolderPath
        {
            get
            {
                return Path.Combine(Application.dataPath, ".Mods");
            }
        }
        #endregion

        #region Static Fields
        private static ModManager instance;
        #endregion

        #region Properties
        public List<Mod> LoadedMods { get; private set; }
        #endregion

        #region Methods
        public void Initialize()
        {
            for (int i = 0; i < LoadedMods.Count; i++)
            {
                LoadedMods[i].Initialize();
            }
        }

        public void OnPreInitialize()
        {
            LoadMods();
            for (int i = 0; i < LoadedMods.Count; i++)
            {
                LoadedMods[i].OnPreInitialize();
            }
            // Check for dependencies and conflicts and matching game version, hook mods initialization into the game's initialization.
        }

        public void OnPostInitialize()
        {
            for (int i = 0; i < LoadedMods.Count; i++)
            {
                LoadedMods[i].OnPostInitialize();
            }
        }

        private void LoadMod(string modInfoPath)
        {
            if (File.Exists(modInfoPath))
            {
                string json = File.ReadAllText(modInfoPath);
                Mod mod = JsonUtility.FromJson<Mod>(json);
                LoadedMods.Add(mod);
            }
            else
            {
                throw new FileNotFoundException($"[ModManager] Mod Info at {modInfoPath} could not be found!");
            }
        }

        private void LoadMods()
        {
            LoadedMods = new List<Mod>();
            string[] modInfoPaths = GetModInfoPaths();
            foreach (string modInfoPath in modInfoPaths)
            {
                LoadMod(modInfoPath);
            }
        }

        private string[] GetModInfoPaths()
        {
            string[] modFolders = Directory.GetDirectories(ModsFolderPath);
            string[] modInfoPaths = new string[modFolders.Length];
            for (int i = 0; i < modFolders.Length; i++)
            {
                modInfoPaths[i] = Path.Combine(modFolders[i], $"{modFolders[i]}.LCMOD");
            }
            return modInfoPaths;
            
        }
        #endregion
    }
}
