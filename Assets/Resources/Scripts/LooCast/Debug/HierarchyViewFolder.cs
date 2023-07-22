using UnityEngine;
using LooCast.System;
using System.Collections.Generic;
using UnityEngine.UI;
using System;

public class HierarchyViewFolder : HierarchyViewElement
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewFolderPrefab;
    [SerializeField] private GameObject hierarchyViewFilePrefab;

    private IFolderComponent hierarchyFolder;
    private Dictionary<string, HierarchyViewFolder> hierarchyViewFolderChildren;
    private Dictionary<string, HierarchyViewFile> hierarchyViewFileChildren;
    private static List<HierarchyViewFolder> allHierarchyViewFolderChildren = new List<HierarchyViewFolder>();
    private Action initializationAction;
    #endregion

    #region Unity Callbacks
    private void Awake()
    {
        initializationAction = () => 
        {
            allHierarchyViewFolderChildren.Add(this);

            if (hierarchyFolder is null)
            {
                expandButton.interactable = false;
                throw new NullReferenceException("Ich bin ein Hurensohn!");
                // After you are done debugging, go into the prefabs and make the elementContainerPanels inactive by default!
            }
            else
            {
                Debug.Log($"Ich bin kein Hurensohn und mein Name ist '{hierarchyFolder.FolderName}'!");
                expandButton.interactable = true;
            }
        };
    }
    #endregion

    #region Methods
    public void Initialize(IFolderComponent hierarchyFolder)
    {
        base.Initialize(hierarchyFolder.FolderName);
        
        this.hierarchyFolder = hierarchyFolder;
        hierarchyViewFolderChildren = new Dictionary<string, HierarchyViewFolder>();
        hierarchyViewFileChildren = new Dictionary<string, HierarchyViewFile>();

        initializationAction.Invoke();
    }
    #endregion

    #region Overrides
    protected override void InstantiateChildren()
    {
        base.InstantiateChildren();

        foreach (IFolderComponent folder in ((IParent<IFolderComponent>)hierarchyFolder).Children)
        {
            HierarchyViewFolder hierarchyViewFolder = Instantiate(hierarchyViewFolderPrefab, elementContainer.transform).GetComponent<HierarchyViewFolder>();
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();

            hierarchyViewFolder.gameObject.name = folder.FolderName;
            hierarchyViewFolderChildren.Add(folder.FolderName, hierarchyViewFolder);
            hierarchyViewFolder.Initialize(folder);
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();
        }

        foreach (IFileComponent file in ((IParent<IFileComponent>)hierarchyFolder).Children)
        {
            HierarchyViewFile hierarchyViewFile = Instantiate(hierarchyViewFilePrefab, elementContainer.transform).GetComponent<HierarchyViewFile>();
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();

            hierarchyViewFile.gameObject.name = file.FileIdentifier;
            hierarchyViewFileChildren.Add(file.FileIdentifier, hierarchyViewFile);
            hierarchyViewFile.Initialize(file);
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();
        }

        if (hierarchyViewFolderChildren.Count != 0 || hierarchyViewFileChildren.Count != 0)
        {
            hasAnyChildren = true;
        }
    }
    #endregion
}
