using UnityEngine;
using LooCast.System;
using System.Collections.Generic;

public class HierarchyView : MonoBehaviour
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewFolderPrefab;
    [SerializeField] private GameObject hierarchyViewFilePrefab;
    
    private Dictionary<string, HierarchyViewFolder> hierarchyViewFolderChildren;
    private Dictionary<string, HierarchyViewFile> hierarchyViewFileChildren;
    #endregion

    #region Unity Callbacks
    private void Start()
    {
        Initialize();
    }
    #endregion

    #region Methods
    public void Initialize()
    {
        hierarchyViewFolderChildren = new Dictionary<string, HierarchyViewFolder>();
        hierarchyViewFileChildren = new Dictionary<string, HierarchyViewFile>();
        
        foreach (IFolder folder in MainManager.Instance.FolderChildren)
        {
            HierarchyViewFolder hierarchyViewFolder = Instantiate(hierarchyViewFolderPrefab, transform).GetComponent<HierarchyViewFolder>();
            hierarchyViewFolder.gameObject.name = folder.FolderName;
            hierarchyViewFolderChildren.Add(folder.FolderName, hierarchyViewFolder);
        }

        foreach (IFile file in MainManager.Instance.FileChildren)
        {
            HierarchyViewFile hierarchyViewFile = Instantiate(hierarchyViewFilePrefab, transform).GetComponent<HierarchyViewFile>();
            hierarchyViewFile.gameObject.name = file.FileName;
            hierarchyViewFileChildren.Add(file.FileName, hierarchyViewFile);
        }

        if (hierarchyViewFolderChildren.Count == 0 && hierarchyViewFileChildren.Count == 0)
        {
            gameObject.SetActive(false);
        }

        foreach (IFolder folder in MainManager.Instance.FolderChildren)
        {
            hierarchyViewFolderChildren.TryGetValue(folder.FolderName, out HierarchyViewFolder hierarchyViewFolder);
            hierarchyViewFolder.Initialize(folder);
        }

        foreach (IFile file in MainManager.Instance.FileChildren)
        {
            hierarchyViewFileChildren.TryGetValue(file.FileName, out HierarchyViewFile hierarchyViewFile);
            hierarchyViewFile.Initialize(file);
        }
    }
    #endregion
}
