using UnityEngine;
using LooCast.System;
using System.Collections.Generic;
using UnityEngine.UI;

public class HierarchyView : MonoBehaviour
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewFolderPrefab;
    [SerializeField] private GameObject hierarchyViewFilePrefab;
    [SerializeField] private LayoutGroup layoutGroup;
    
    private Dictionary<string, HierarchyViewFolder> hierarchyViewFolderChildren;
    private Dictionary<string, HierarchyViewFile> hierarchyViewFileChildren;
    #endregion

    #region Unity Callbacks
    private void Awake()
    {
        Initialize();
    }
    #endregion

    #region Methods
    public void Initialize()
    {
        hierarchyViewFolderChildren = new Dictionary<string, HierarchyViewFolder>();
        hierarchyViewFileChildren = new Dictionary<string, HierarchyViewFile>();

        FolderComponent rootFolder = MainManager.Instance.GetComponent<FolderComponent>();

        foreach (IFolder folder in rootFolder.FolderChildren)
        {
            HierarchyViewFolder hierarchyViewFolder = Instantiate(hierarchyViewFolderPrefab, transform).GetComponent<HierarchyViewFolder>();
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            layoutGroup.CalculateLayoutInputHorizontal();
            layoutGroup.CalculateLayoutInputVertical();

            hierarchyViewFolder.gameObject.name = folder.FolderName;
            hierarchyViewFolderChildren.Add(folder.FolderName, hierarchyViewFolder);
            hierarchyViewFolder.Initialize(folder);
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            layoutGroup.CalculateLayoutInputHorizontal();
            layoutGroup.CalculateLayoutInputVertical();
        }

        foreach (IFile file in rootFolder.FileChildren)
        {
            HierarchyViewFile hierarchyViewFile = Instantiate(hierarchyViewFilePrefab, transform).GetComponent<HierarchyViewFile>();
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            layoutGroup.CalculateLayoutInputHorizontal();
            layoutGroup.CalculateLayoutInputVertical();

            hierarchyViewFile.gameObject.name = file.FileName;
            hierarchyViewFileChildren.Add(file.FileName, hierarchyViewFile);
            hierarchyViewFile.Initialize(file);
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            layoutGroup.CalculateLayoutInputHorizontal();
            layoutGroup.CalculateLayoutInputVertical();
        }

        if (hierarchyViewFolderChildren.Count == 0 && hierarchyViewFileChildren.Count == 0)
        {
            gameObject.SetActive(false);
        }
    }
    #endregion
}
