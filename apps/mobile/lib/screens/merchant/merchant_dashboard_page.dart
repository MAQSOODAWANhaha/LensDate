import "package:flutter/material.dart";

import "merchant_approval_list_page.dart";
import "merchant_asset_list_page.dart";
import "merchant_contract_list_page.dart";
import "merchant_invoice_list_page.dart";
import "merchant_location_list_page.dart";
import "merchant_member_list_page.dart";
import "merchant_order_list_page.dart";
import "merchant_template_list_page.dart";

class MerchantDashboardPage extends StatelessWidget {
  final Map<String, dynamic> merchant;
  const MerchantDashboardPage({super.key, required this.merchant});

  int get merchantId => merchant["id"] as int? ?? 0;
  String get merchantName => merchant["name"]?.toString() ?? "商户";

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(merchantName)),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          ListTile(
            leading: const Icon(Icons.receipt_long),
            title: const Text("商户订单"),
            subtitle: const Text("查看与该商户关联的订单"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantOrderListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.store_mall_directory_outlined),
            title: const Text("门店管理"),
            subtitle: const Text("维护门店信息"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantLocationListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.group_outlined),
            title: const Text("成员管理"),
            subtitle: const Text("维护商户成员与角色"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantMemberListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.inventory_2_outlined),
            title: const Text("套餐模板"),
            subtitle: const Text("维护拍摄套餐与条目"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantTemplateListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.palette_outlined),
            title: const Text("品牌素材库"),
            subtitle: const Text("管理 Logo、风格与参考素材"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantAssetListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.approval_outlined),
            title: const Text("审批流程"),
            subtitle: const Text("管理审批记录"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantApprovalListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.description_outlined),
            title: const Text("合同管理"),
            subtitle: const Text("查看合同条款"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantContractListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.receipt),
            title: const Text("发票管理"),
            subtitle: const Text("开票与收款记录"),
            onTap: () => Navigator.of(context).push(
              MaterialPageRoute(
                builder: (_) => MerchantInvoiceListPage(
                  merchantId: merchantId,
                  merchantName: merchantName,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
