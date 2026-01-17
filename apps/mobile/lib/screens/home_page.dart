import "package:flutter/material.dart";

import "demands/demand_list_page.dart";
import "orders/order_list_page.dart";
import "messages_page.dart";
import "profile_page.dart";
import "../services/notification_store.dart";

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int _index = 0;

  final List<Widget> _pages = const [
    DemandListPage(),
    OrderListPage(),
    MessagesPage(),
    ProfilePage(),
  ];

  @override
  void initState() {
    super.initState();
    NotificationStore.refresh();
  }

  Widget _buildBadgeIcon(IconData icon, int count) {
    return Stack(
      clipBehavior: Clip.none,
      children: [
        Icon(icon),
        if (count > 0)
          Positioned(
            right: -6,
            top: -4,
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 1),
              decoration: BoxDecoration(
                color: Colors.red,
                borderRadius: BorderRadius.circular(10),
              ),
              constraints: const BoxConstraints(minWidth: 16),
              child: Text(
                count > 99 ? "99+" : "$count",
                style: const TextStyle(color: Colors.white, fontSize: 10),
                textAlign: TextAlign.center,
              ),
            ),
          ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<int>(
      valueListenable: NotificationStore.unreadCount,
      builder: (context, unreadCount, _) {
        return Scaffold(
          body: IndexedStack(index: _index, children: _pages),
          bottomNavigationBar: BottomNavigationBar(
            currentIndex: _index,
            onTap: (value) {
              setState(() => _index = value);
              if (value == 2) {
                NotificationStore.refresh();
              }
            },
            items: [
              const BottomNavigationBarItem(icon: Icon(Icons.assignment), label: "需求"),
              const BottomNavigationBarItem(icon: Icon(Icons.receipt_long), label: "订单"),
              BottomNavigationBarItem(
                icon: _buildBadgeIcon(Icons.chat_bubble_outline, unreadCount),
                label: "消息",
              ),
              const BottomNavigationBarItem(icon: Icon(Icons.person_outline), label: "我的"),
            ],
          ),
        );
      },
    );
  }
}
